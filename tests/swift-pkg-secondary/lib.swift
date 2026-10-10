import SwiftRs

@_cdecl("echo_data_secondary")
func echoDataSecondary(data: SRData) -> SRData {
    return SRData(data.toArray())
}
