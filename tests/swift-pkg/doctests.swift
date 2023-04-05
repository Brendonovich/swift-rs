import Foundation
import SwiftRs

// SRArray
//
// Notice that IntArray and ArrayStruct are almost identical!
// The only actual difference between these types is how they're used in Rust,
// but if you added more fields to ArrayStruct then that wouldn't be the case anymore.

class IntArray: NSObject {
    var data: SRArray<Int>

    init(data: [Int]) {
        self.data = SRArray(data)
    }
}

@_cdecl("get_int_array")
func getIntArray() -> IntArray {
    return IntArray(data: [1, 2, 3])
}

class ArrayStruct: NSObject {
    var array: SRArray<Int>

    init(array: [Int]) {
        self.array = SRArray(array)
    }
}

@_cdecl("get_array_struct")
func getArrayStruct() -> ArrayStruct {
    return ArrayStruct(array: [4, 5, 6])
}

// SRObject

class CustomObject: NSObject {
    var a: Int
    var b: Bool

    init(a: Int, b: Bool) {
        self.a = a
        self.b = b
    }
}

@_cdecl("get_custom_object")
func getCustomObject() -> CustomObject {
    return CustomObject(a: 3, b: true)
}

// SRString

@_cdecl("get_greeting")
func getGreeting(name: SRString) -> SRString {
    return SRString("Hello \(name.toString())!")
}

@_cdecl("echo")
func echo(string: SRString) -> SRString {
    return string
}

// SRData

@_cdecl("get_data")
func getData() -> SRData {
    return SRData([1, 2, 3])
}
