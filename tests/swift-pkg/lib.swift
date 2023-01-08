import SwiftRs

@_cdecl("get_greeting")
func getGreeting(name: SRString) -> SRString {
    return SRString("Hello \(name)")
}
