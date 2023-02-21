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

class Complex: NSObject {
    var a: SRString
    var b: Int
    var c: Bool
    
    public init(a: SRString, b: Int, c: Bool) {
        self.a = a
        self.b = b
        self.c = c
    }
}

@_cdecl("complex_data")
func complexData() -> SRObjectArray {
    return SRObjectArray([
        Complex(a: SRString("Brendan"), b: 0, c: true),
        Complex(a: SRString("Amod"), b: 1, c: false),
        Complex(a: SRString("Lucas"), b: 2, c: true),
        Complex(a: SRString("Oscar"), b: 3, c: false),
    ])
}
