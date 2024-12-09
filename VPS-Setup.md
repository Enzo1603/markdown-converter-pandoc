# VPS Setup

- Server: Der VPS-Server bei IONOS
- Client: Gerät im Heimnetz (z. B. RaspberryPi)

## Firewall

Gehe auf der IONOS-Webseite zu `Netzwerk > Firewall-Richtlinien`: Füge deinem Server eine IPv6-Adresse hinzu (IPv4 sollte bereits zugewiesen sein).
Setze mindestens folgende Regeln:

- TCP auf Port 22 (SSH)
- TCP auf Port 80 (HTTP, Nginx Proxy Manager)
- TCP auf Port 443 (HTTPS, Nginx Proxy Manager)
- UDP auf Port 51820 (WireGuard Tunnel)
- UDP z. B. auf Port 5544 (WireGuard VPN)

## Erstes Login und ändern des Default-Root-Passworts

Melde dich über SSH an

```bash
ssh root@<vps-ip-adresse>
```

Ändere das Passwort

```bash
passwd
```

## System aktualisieren

```bash
sudo apt update
sudo apt upgrade
```

## Erstelle neuen Benutzer und deaktiviere Root-Login über SSH

Neuen Benutzer erstellen

```bash
adduser <username>
```

Füge den Benutzer der sudo-Gruppe hinzu

```bash
usermod -aG sudo <username>
```

Deaktiviere Root-Login über SSH. Ändere folgende Zeile oder füge sie der Datei `/etc/ssh/sshd_config` hinzu

```bash
PermitRootLogin no
```

Starte den SSH-Dienst neu

```bash
sudo systemctl restart ssh
```

Logge den Root aus und melde dich mit deinem neuen User an

```bash
exit

ssh <username>@<vps-ip-adresse>
```

## Passwortloses SSH-Login

Kopiere deinen SSH-Key auf den Server, damit du dich ohne Passwort von deinem PC/Laptop aus einloggen kannst muss natürlich für jedes Gerät einzeln gemacht werden :(. (Gegebenenfalls muss zuerst noch ein SSH key generiert werden.)

```bash
ssh-copy-id <username>@<vps_ip_adresse>
```

## Installiere WireGuard und richte es für den Tunnel ein

Stelle sicher, dass der Port in der IONOS-Firewall freigegeben ist!

Installiere WireGuard

```bash
sudo apt install wireguard
```

Aktiviere das IP-Forwarding. Kommentiere folgende Zeilen aus in der Datei `/etc/sysctl.conf`

```bash
net.ipv4.ip_forward=1
# optional für IPv6 (für IPv6 muss gegebenenfalls dem VPS eine extra IPv6 noch zugewiesen werden über das IONOS Portal!)
net.ipv6.conf.all.forwarding=1  # Enabling this option disables Stateless Address Autoconfiguration based on Router Advertisements for this host
```

Lade die neuen IP-Forwarding-Einstellungen.

```bash
sudo sysctl -p
```

Wechsle in `/etc/wireguard` und erstelle die WireGuard Schlüsselpaare für den Server

```bash
wg genkey | tee server_private.key | wg pubkey > server_public.key
```

Erstelle `/etc/wireguard/wg0.conf`, um den WireGuard-Tunnel auf dem Server zu konfigurieren

```bash
sudo nano /etc/wireguard/wg0.conf
```

Füge die entsprechende Konfiguration ein. Ersetze <Server_Private_Key> mit dem Key in der Datei `server_private.key`. (<Client_Public_Key> kommt später.) **Wichtig: Ersetze 'ens6' mit deinem gewünschten Network Interface (kann z. B. auch 'eth0' heissen; kann mit `ip addr` herausgefunden werden).**

```bash
[Interface]
PrivateKey = <Server_Private_Key>
Address = 10.0.0.1/24, fd86:ea04:1115::1/64  # IPv6 optional
ListenPort = 51820
MTU = 1400   # WICHTIG (sollte auf jeden Fall unter 1500 sein)

# IP-Forwarding aktivieren (IPv4)
PostUp = iptables -A FORWARD -i wg0 -j ACCEPT; iptables -t nat -A POSTROUTING -o ens6 -j MASQUERADE
PostDown = iptables -D FORWARD -i wg0 -j ACCEPT; iptables -t nat -D POSTROUTING -o ens6 -j MASQUERADE

# IP-Forwarding aktivieren (IPv6) (optional)
PostUp = ip6tables -A FORWARD -i wg0 -j ACCEPT; ip6tables -t nat -A POSTROUTING -o ens6 -j MASQUERADE
PostDown = ip6tables -D FORWARD -i wg0 -j ACCEPT; ip6tables -t nat -D POSTROUTING -o ens6 -j MASQUERADE

[Peer]
PublicKey = <Client_Public_Key>
AllowedIPs = 10.0.0.0/24, 192.168.1.0/24, fd86:ea04:1115::/64   # IPv6 optional
```

# Client Setup

## Installiere WireGuard und richte es für den Tunnel ein

Installiere WireGuard wie zuvor oben...

- `apt install wireguard`
- Editiere `/etc/sysctl.conf`, um IP-Forwarding zu aktivieren
- `sudo sysctl -p`, um die neue IP-Forwarding-Konfiguration zu laden

Erstelle das WireGuard-Schlüsselpaar für den Client

```bash
cd /etc/wireguard

wg genkey | tee client_private.key | wg pubkey > client_public.key
```

Erstelle `/etc/wireguard/wg0.conf`, um WireGuard auf dem Client für den WireGuard-Tunnel zu konfigurieren

```bash
sudo nano /etc/wireguard/wg0.conf
```

Füge die entsprechende Konfiguration ein. Ersetze <Client_Private_Key> mit dem Key in der Datei `client_private.key`. Ersetze <Server_Public_Key> mit dem Key in der Datei `server_public.key` auf dem Server. Ersetze <VPS_IPv4_Adresse> mit der öffentlichen IPv4-Adresse deines VPS-Servers. **Wichtig: Ändere gegebenenfalls `eth0` zum korrekten Netzwerk-Interface.**

```bash
[Interface]
PrivateKey = <Client_Private_Key>
Address = 10.0.0.2/24, fd86:ea04:1115::2/64  # IPv6 optional
ListenPort = 51820
MTU = 1400   # WICHTIG (sollte auf jeden Fall unter 1500 sein)


# IP-Forwarding aktivieren (IPv4)
PostUp = iptables -A FORWARD -i wg0 -j ACCEPT; iptables -t nat -A POSTROUTING -o eth0 -j MASQUERADE
PostDown = iptables -D FORWARD -i wg0 -j ACCEPT; iptables -t nat -D POSTROUTING -o eth0 -j MASQUERADE

# IP-Forwarding aktivieren (IPv6) (optional)
PostUp = ip6tables -A FORWARD -i wg0 -j ACCEPT; ip6tables -t nat -A POSTROUTING -o eth0 -j MASQUERADE
PostDown = ip6tables -D FORWARD -i wg0 -j ACCEPT; ip6tables -t nat -D POSTROUTING -o eth0 -j MASQUERADE


[Peer]
PublicKey = <Server_Public_Key>
Endpoint = <VPS_IPv4_Adresse>:51820
AllowedIPs = 10.0.0.0/24, fd86:ea04:1115::/64  # IPv6 optional
PersistentKeepalive = 20
```

## Public Client Key auf Server wg0.conf

Füge den Public-Key des Clients noch in die `/etc/wireguard/wg0.conf` deines Servers hinzu, da du sie jetzt kennst!

## Aktiviere WireGuard sowohl auf dem Server als dem Client

Aktiviere und starte WireGuard (auf dem Client **und** dem Server)

```bash
sudo systemctl enable wg-quick@wg0
sudo systemctl start wg-quick@wg0
```

Überprüfe, ob der Tunnel erfolgreich aufgebaut worden ist ("last handshake" sollte angezeigt werden, sonst stimmt was nicht).

```bash
sudo wg show
```

# Installation des Nginx Proxy Managers

## Installiere Docker auf dem VPS-Server

[Debian Docker Installation](https://docs.docker.com/engine/install/debian/)

## Installiere Nginx Proxy Manager

Erstelle den Ordner `~/npm`

```bash
mkdir ~/npm

cd ~/npm
```

Erstelle `docker-compose.yml` und kopiere folgenden Inhalt hinein

```yaml
services:
  app:
    image: jc21/nginx-proxy-manager:latest
    container_name: npm
    restart: always
    ports:
      - 80:80
      - 81:81
      - 443:443
    volumes:
      - /PASS/MICH/AN/npm/data:/data
      - /PASS/MICH/AN/npm/letsencrypt:/etc/letsencrypt
```

Starte den Container mit

```bash
sudo docker compose up -d
```

Leite Anfragen zum Client auf Port 82 über den WireGuard-Tunnel zum VPS auf Port 81 (damit Nginx Proxy Manager für die Erstkonfiguration von temporär vom Heimnetz aus erreichbar ist). Achtung: Das ist nicht persistent, nach einem Neustart ist diese Konfiguration wieder weg. Füge `npm.baraldi.ch` auf dem Proxy Manager hinzu, damit man ihn ab sofort über die URL erreichen kann. (Die Konfiguration unten wird danach nicht mehr beötigt und verschwindet automatisch nach einem Neustart wieder.)

```bash
# Für IPv4
sudo iptables -t nat -A PREROUTING -p tcp --dport 82 -j DNAT --to-destination 10.0.0.1:81

sudo iptables -t nat -A POSTROUTING -j MASQUERADE
```

# WireGuard VPN über den VPS

Stelle sicher, dass in der IONOS-Firewall der entsprechende Port über UDP freigegeben ist. Sage dem VPS, er soll alles auf Port 5544 über UDP an 192.168.1.20:5544 weiterleiten (Heimnetz WireGuard Docker Container).

```bash
sudo iptables -t nat -A PREROUTING -p udp --dport 5544 -j DNAT --to-destination 192.168.1.20:5544

sudo iptables -t nat -A POSTROUTING -j MASQUERADE
```

Installiere iptables-persistent, um die iptables configs speichern zu können (zweimal 'yes'), damit sie auch nach dem Neustart noch erhalten sind.

```bash
sudo apt install iptables-persistent
```

Normalerweise wird man bei der Installation gefragt, ob man die aktuelle Konfiguration speichern möchte. Manuell hiermit (am einfachsten unter `sudo su` ausführen):

```bash
sudo iptables-save > /etc/iptables/rules.v4
sudo ip6tables-save > /etc/iptables/rules.v6   # optional
```

Setze den Docker Container auf

```yaml
volumes:
  etc_wireguard:

services:
  wg-easy:
    environment:
      # Change Language:
      # (Supports: en, ua, ru, tr, no, pl, fr, de, ca, es, ko, vi, nl, is, pt, chs, cht, it, th, hi, ja, si)
      - LANG=de
      # ⚠️ Required:
      # Change this to your host's public address
      - WG_HOST=87.106.70.163

      # Optional:
      # - PASSWORD_HASH=$$2y$$10$$hBCoykrB95WSzuV4fafBzOHWKu9sbyVa34GJr8VV5R/pIelfEMYyG (needs double $$, hash of 'foobar123'; see "How_to_generate_an_bcrypt_hash.md" for generate the hash)
      - PORT=5545
      - WG_PORT=5544
      # - WG_CONFIG_PORT=92820
      # - WG_DEFAULT_ADDRESS=10.8.0.x
      # - WG_DEFAULT_DNS=1.1.1.1
      # - WG_MTU=1420
      # - WG_ALLOWED_IPS=192.168.15.0/24, 10.0.1.0/24
      # - WG_PERSISTENT_KEEPALIVE=25
      # - WG_PRE_UP=echo "Pre Up" > /etc/wireguard/pre-up.txt
      # - WG_POST_UP=echo "Post Up" > /etc/wireguard/post-up.txt
      # - WG_PRE_DOWN=echo "Pre Down" > /etc/wireguard/pre-down.txt
      # - WG_POST_DOWN=echo "Post Down" > /etc/wireguard/post-down.txt
      # - UI_TRAFFIC_STATS=true
      # - UI_CHART_TYPE=0 # (0 Charts disabled, 1 # Line chart, 2 # Area chart, 3 # Bar chart)
      # - WG_ENABLE_ONE_TIME_LINKS=true
      # - UI_ENABLE_SORT_CLIENTS=true
      # - WG_ENABLE_EXPIRES_TIME=true
      # - ENABLE_PROMETHEUS_METRICS=false
      # - PROMETHEUS_METRICS_PASSWORD=$$2a$$12$$vkvKpeEAHD78gasyawIod.1leBMKg8sBwKW.pQyNsq78bXV3INf2G # (needs double $$, hash of 'prometheus_password'; see "How_to_generate_an_bcrypt_hash.md" for generate the hash)

    image: ghcr.io/wg-easy/wg-easy
    container_name: wg-easy
    volumes:
      - etc_wireguard:/etc/wireguard
    ports:
      - "5544:5544/udp"
      - "5545:5545/tcp"
    restart: unless-stopped
    cap_add:
      - NET_ADMIN
      - SYS_MODULE
      # - NET_RAW # ⚠️ Uncomment if using Podman
    sysctls:
      - net.ipv4.ip_forward=1
      - net.ipv4.conf.all.src_valid_mark=1
```
