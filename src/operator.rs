use subscriber::{ Subscriber };
use types::{ TeardownLogic };

pub struct Operator<T, R> {
  call(subscriber: Subscriber<R>, source: any): TeardownLogic;
}
