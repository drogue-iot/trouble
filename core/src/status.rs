
pub enum Status {
    Success = 0x00,
    UnknownHciCommand = 0x01,
    UnknownConnectionIdentifier = 0x02,
    HardwareFailure = 0x03,
    PageTimeout = 0x04,
    AuthenticationFailure = 0x05,
    PinOrKeyMissing = 0x06,
    MemoryCapacityExceeded = 0x07,
    ConnectionTimeout = 0x08,
    ConnectionLimitExceeded = 0x09,
    SynchronousConnectionLimitToADeviceExceeded = 0x0A,
    ConnectionAlreadyExists = 0x0B,
    CommandDisallowed = 0x0C,
    ConnectionRejectedDueToLimitedResources = 0x0D,
    ConnectionRejectedDueToSecurityReasons = 0x0E,
    ConnectionRejectedDueToUnacceptableBdAddr = 0x0F,
    ConnectionAcceptTimeoutExceeded = 0x10,
    UnsupportedFeatureOrParameterValue = 0x11,
    InvalidHciCommandParameters = 0x12,
    RemoteUserTerminatedConnection = 0x13,
    RemoteDeviceTerminatedConnectionDueToLoweResources = 0x14,
    RemoteDeviceTerminatedConnectionDueToPowerOff = 0x15,
    ConnectionTerminatedByLocalHost = 0x16,
    RepeatedAttempts = 0x17,
    PairingNotAllowed = 0x18,
    UnknownLmpPdu = 0x19,
    UnsupportedRemoteFeature = 0x1A,
    ScoOffsetRejected = 0x1B,
    ScoINtervalRejected = 0x1D,
    ScoAirModeRejected = 0x1E,
    UnspecifiedError = 0x1F,
    UnsupportedLmpParameterValue = 0x20,
    RoleChangeNotAllowed = 0x21,
    LmpResponseTimeout = 0x22,
    LmpErrorTransactionCollision = 0x23,
    LmpPduNotAllowed = 0x24,
    EncryptionModeNotAccetable = 0x25,
    LinkKeyCannotBeChanged = 0x26,
    RequestedQoSNotSupported = 0x27,
    InstantPassed = 0x28,
    PairingWithUnitKeyNotSupported = 0x29,
    DifferentTranactionCollision = 0x2A,
    //ReservedForFutureUse = 0x2B,
    QoSUnacceptableParameter = 0x2C,
    QoSRejected = 0x2D,
    ChannelClassificationNotSupported = 0x2E,
    InsufficientSecurity = 0x2F,
    ParameterOutOfMandatoryRange = 0x30,
    //ReservedForFutureUser = 0x31,
    RoleSwitchPending = 0x32,
    //ReservedForFutureUser = 0x33,
    ReservedSlotViolation = 0x34,
    RoleSwitchFailed = 0x35,
    ExtendedInquiryResponseTooLarge = 0x36,
    SecureSimplePairingNotSupportedByHost = 0x37,
    HostBusyPairing = 0x38,
    ConnectionRejectedDueToNotSuitableChannelFound = 0x39,
    ControllerBusy = 0x3A,
    UnacceptableConnectionParameters = 0x3B,
    AdvertisingTimeout = 0x3C,
    ConnectionTerminatedDueToMicFailure = 0x3D,
    ConnectionFailedToBeEstablished = 0x3E,
    MacConnectionFailed = 0x3F,
    CoardClockAdjustedRejectedWillTryDragging = 0x40,
    Type0SubmapNotDefined = 0x41,
    UnknownAdvertisingIdentifier = 0x42,
    LimitReached = 0x43,
    OperationCancelledByHost = 0x44,
    PacketTooLong = 0x45,
}