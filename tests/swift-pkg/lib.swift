import SwiftRs
import Foundation

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

@_cdecl("echo_data")
func echoData(data: SRData) -> SRData {
    return SRData(data.toArray())
}
