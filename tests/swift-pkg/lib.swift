import SwiftRs
import Foundation

@_cdecl("get_greeting")
func getGreeting(name: SRString) -> SRString {
    return SRString("Hello \(name.to_string())");
}

@_cdecl("reflect_string")
func reflectString(str: SRString) -> SRString {
    return str
}

@_cdecl("retain_count")
func retainCount(str: SRString) {
    print("\(CFGetRetainCount(str))")
}
