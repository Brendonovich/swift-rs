import Foundation

public class SRArray<T>: NSObject, ExpressibleByArrayLiteral {
    public typealias ArrayLiteralElement = T
    
    // Used by Rust
    let pointer: UnsafePointer<T>
    let length: Int
    
    // Actual array, deallocates objects inside automatically
    let array: [T]

    public override init() {
        self.array = []
        self.pointer = UnsafePointer(self.array)
        self.length = 0
    }

    public init(_ data: [T]) {
        self.array = data
        self.pointer = UnsafePointer(self.array)
        self.length = data.count
    }
    
    public required init(arrayLiteral elements: T...) {
        self.array = elements
        self.pointer = UnsafePointer(elements)
        self.length = elements.count
    }

    public func toArray() -> [T] {
        return self.array
    }
}

public class SRObjectArray: NSObject {
    let data: SRArray<NSObject>
    
    public init(_ data: [NSObject]) {
        self.data = SRArray(data)
    }
}

public class SRData: NSObject {
    let data: SRArray<UInt8>
    
    public override init() {
        self.data = SRArray()
    }
    
    public init(_ data: [UInt8]) {
        self.data = SRArray(data)
    }
    
    public init (_ srArray: SRArray<UInt8>) {
        self.data = srArray
    }

    public func toArray() -> [UInt8] {
        return self.data.toArray()
    }
}

public class SRString: SRData, ExpressibleByStringLiteral {
    public typealias StringLiteralType = String
    
    public override init() {
        super.init([])
    }

    public init(_ string: String) {
        super.init(Array(string.utf8))
    }

    public required init(stringLiteral value: String) {
        super.init(Array(value.utf8))
    }
    
    init(_ data: SRData) {
        super.init(data.data)
    }

    public func toString() -> String {
        return String(bytes: self.data.array, encoding: .utf8)!
    }
}

@_cdecl("retain_object")
func retainObject(ptr: UnsafeMutableRawPointer) {
    let _ = Unmanaged<AnyObject>.fromOpaque(ptr).retain()
}

@_cdecl("release_object")
func releaseObject(ptr: UnsafeMutableRawPointer) {
    let _ = Unmanaged<AnyObject>.fromOpaque(ptr).release()
}

@_cdecl("data_from_bytes")
func dataFromBytes(data: UnsafePointer<UInt8>, size: Int) -> SRData {
    let buffer = UnsafeBufferPointer(start: data, count: size)
    return SRData(Array(buffer))
}

@_cdecl("string_from_bytes")
func stringFromBytes(data: UnsafePointer<UInt8>, size: Int) -> SRString {
    let data = dataFromBytes(data: data, size: size);
    return SRString(data)
}
