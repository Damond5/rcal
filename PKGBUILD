pkgname=rcal
pkgver=0.1.0
pkgrel=1
pkgdesc="A terminal-based calendar application built with Rust and Ratatui."
arch=('x86_64')
url="https://github.com/Damond5/rcal"
license=('CC0-1.0')
depends=()
makedepends=('cargo' 'rust')
source=("$pkgname::git+$url.git#branch=main")
sha256sums=('SKIP')

build() {
    cd "$pkgname"
    cargo build --release
}

package() {
    cd "$pkgname"
    install -Dm755 target/release/rcal "$pkgdir/usr/bin/rcal"
    install -Dm644 LICENSE "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
    # Create systemd user service for daemon
    mkdir -p "$pkgdir/usr/lib/systemd/user"
    cat > "$pkgdir/usr/lib/systemd/user/rcal.service" << EOF
[Unit]
Description=rcal notification daemon
After=network.target

[Service]
ExecStart=/usr/bin/rcal --daemon
Restart=always
RestartSec=5

[Install]
WantedBy=default.target
EOF
}