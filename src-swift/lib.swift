import Foundation

public class SRArray<T>: NSObject {
    public var pointer: UnsafePointer<T>
    public var length: Int
    
    public override init() {
        self.pointer = UnsafePointer([]);
        self.length = 0;
    }
    
    public init(_ data: [T]) {
        let mut_data = UnsafeMutablePointer<T>.allocate(capacity: data.count)
        mut_data.initialize(from: data, count: data.count)
        
        self.pointer = UnsafePointer(mut_data)
        self.length = data.count
    }
}

public class SRObjectArray: NSObject {
    var data: SRArray<NSObject>
    
    public init(_ data: [NSObject]) {
        self.data = SRArray(data)
    }
}

public class SRData: NSObject {
    public var data: SRArray<UInt8>
    
    public override init() {
        self.data = SRArray<UInt8>.init()
    }
    
    public init(_ data: [UInt8]) {
        self.data = SRArray(data)
    }
    
    public func to_data() -> Data {
        return Data(bytes: self.data.pointer, count: self.data.length)
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
        return String(bytes: self.to_data(), encoding: .utf8)!
    }
    
    deinit {
        print("Deinit string \(self.to_string())")
    }
}

@_cdecl("allocate_string")
public func allocateString(data: UnsafePointer<UInt8>, size: Int) -> SRString {
    let buffer = UnsafeBufferPointer(start: data, count: size)
    let string = String(bytes: buffer, encoding: .utf8)!
    let ret = SRString(string)
    return ret
}
