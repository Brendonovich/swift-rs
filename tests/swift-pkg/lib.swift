import SwiftRs

@_cdecl("get_greeting")
func getGreeting(input: SRString) -> SRString {
    return SRString("Hello \(input.to_string())")
}
