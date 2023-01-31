import Foundation

public class SRArray<T>: NSObject {
    // Used by Rust
    let pointer: UnsafePointer<T>
    let length: Int;
    
    // Actual array, deallocates objects inside automatically
    let array: [T];

    public override init() {
        self.array = [];
        self.pointer = UnsafePointer(self.array);
        self.length = 0;
    }

    public init(_ data: [T]) {
        self.array = data;
        self.pointer = UnsafePointer(self.array)
        self.length = data.count
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
}

public class SRString: SRData {
    public override init() {
        super.init([])
    }

    public init(_ string: String) {
        super.init(Array(string.utf8))
    }

    public func to_string() -> String {
        return String(bytes: self.data.array, encoding: .utf8)!
    }
}

/**
 Prepares `obj` to be sent to Rust. This takes a retained copy of `obj`,
 converts it into a raw pointer, and then returns a ref to this pointer.
 The responsibility of releasing this object from hereon, lies with Rust.

 - Returns: a reference to the same object
 */
public func toRust<T: NSObject>(_ obj: T) -> T {
    return obj.toRust()
}

extension NSObject {
    /**
     Prepares `obj` to be sent to Rust. This takes a retained copy of `obj`,
     converts it into a raw pointer, and then returns a ref to this pointer.
     The responsibility of releasing this object from hereon, lies with Rust.

     - Returns: a reference to the same object
     */
    public func toRust<T: NSObject>() -> T {
        let ownedPtr = Unmanaged.passRetained(self).toOpaque()
        return Unmanaged<T>.fromOpaque(ownedPtr).takeUnretainedValue()
    }
}

@_cdecl("allocate_string")
func allocateString(data: UnsafePointer<UInt8>, size: Int) -> SRString {
    let buffer = UnsafeBufferPointer(start: data, count: size)
    let string = String(bytes: buffer, encoding: .utf8)!
    return SRString(string).toRust()
}

@_cdecl("release_object")
func releaseObject(ptr: UnsafeMutableRawPointer) {
    let _ = Unmanaged<AnyObject>.fromOpaque(ptr).release()
}
