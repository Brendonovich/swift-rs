import SwiftRs

@_cdecl("get_greeting")
func getGreeting(name: UnsafePointer<SRString>) -> SRString {
    return toRust(SRString("Hello \(name.pointee.to_string())"))
}

@_cdecl("reflect_string")
func reflectString(ptr: UnsafePointer<SRString>) -> SRString {
    return ptr.pointee
}