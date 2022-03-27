use swift_rs::build;

fn main() {
    build::link_swift();
    build::link_swift_package("swift-lib", "./swift-lib/");
}
