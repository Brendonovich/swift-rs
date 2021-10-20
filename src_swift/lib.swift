import Foundation
import AppKit

// Size: 24 bytes
public class SRArray<T>: NSObject {
    var pointer: UnsafePointer<T>
    var length: Int
    
    public init(_ data: [T]) {
        let mut_data = UnsafeMutablePointer<T>.allocate(capacity: data.count)
        mut_data.initialize(from: data, count: data.count)
        
        self.pointer = UnsafePointer(mut_data)
        self.length = data.count
    }
}

// Size: 16 bytes
public class SRData: NSObject {
    var data: SRArray<UInt8>

    public init(_ data: [UInt8]) {
        self.data = SRArray(data)
    }
    
    public func to_data() -> Data {
        return Data(bytes: self.data.pointer, count: self.data.length)
    }
}

// Size: 16 bytes
public class SRString: SRData {
    public init(_ string: String) {
        super.init(Array(string.utf8))
    }

    public func to_string() -> String {
        return String(bytes: self.to_data(), encoding: .utf8)!
    }
}

// SRstring pointer is passed to rust correctly
// data pointer is passed to rust correctly
// guessing that the type of SRArray isn't the same
@_cdecl("allocate_string")
public func allocate_string(data: UnsafePointer<UInt8>, size: Int) -> SRString {
    let buffer = UnsafeBufferPointer(start: data, count: size)
    let string = String(bytes: buffer, encoding: .utf8)!;
    let SRstring = SRString(string);
    return SRstring
}