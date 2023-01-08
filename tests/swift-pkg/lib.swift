import SwiftRs

@_cdecl("get_greeting")
func getGreeting(name: UnsafePointer<SRString>) -> SRString {
    return toRust(SRString("Hello \(name.pointee.to_string())"))
}
