# Maintainer: Julius de Jeu <julius@voidcorp.nl>
pkgname=sudors
pkgver=0.1.0
pkgrel=1
pkgdesc="A (very good) implementation of sudo but it's in rust"
makedepends=('cargo')
depends=('gcc-libs')
arch=('i686' 'x86_64' 'armv6h' 'armv7h')
url=https://github.com/J00LZZ/SudoRS
license=('Apache')

build() {
    cargo build --release
}

package() {
    cd ..
    install -Dm 4755 target/release/${pkgname} -t "${pkgdir}/usr/bin"

    install -Dm644 "config.default.toml" "${pkgdir}/etc/sudors.toml"
}
