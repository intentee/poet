#[derive(Debug, Eq, PartialEq)]
pub enum NotificationDelivery {
    Delivered,
    EventStreamClosed,
}
