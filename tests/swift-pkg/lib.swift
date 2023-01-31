import SwiftRs

@_cdecl("get_greeting")
func getGreeting(name: SRString) -> SRString {
    return SRString("Hello \(name.to_string())").toRust()
}

@_cdecl("reflect_string")
func reflectString(str: SRString) -> SRString {
    return str
}