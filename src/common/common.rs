#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AccountType {
    Demo = 0,
    Live = 1,
}

impl AccountType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Demo => "Demo",
            Self::Live => "live",
        }
    }
}

impl ToString for AccountType {
    fn to_string(&self) -> String {
        self.as_str().to_string()
    }
}


#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OrderStatus {
    Pending = 0,
    Processing = 1,
    OnHold = 2,
    Completed = 3,
    Cancelled= 4,
    Refunded = 5,
    Failed = 6,
    Trash = 7,
}

impl OrderStatus {
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OrderStatus::Pending => "Pending",
            OrderStatus::Processing => "Processing",
            OrderStatus::OnHold => "OnHold",
            OrderStatus::Completed => "Completed",
            OrderStatus::Cancelled => "Cancelled",
            OrderStatus::Refunded => "Refunded",
            OrderStatus::Failed => "Failed",
            OrderStatus::Trash => "Trash",
        }
    }
}