// import { Observable } from './Observable';
// import { Subscription } from './Subscription';

/** OPERATOR INTERFACES */
// pub mod types {
  pub type UnaryFunction<T, R> = Fn(T) -> R;
  
  pub type OperatorFunction<T, R> = UnaryFunction<Observable<T>, Observable<R>>;

  type Value<T> = T;

  type Factory<T> = Fn() -> T;

  enum FactoryOrValue<T> {
    Value,
    Factory
  }

  pub type MonoTypeOperatorFunction<T> = OperatorFunction<T, T>;

  pub struct Timestamp<T> {
    value: T,
    timestamp: usize,
  }

  pub struct TimeInterval<T> {
    value: T,
    interval: usize,
  }

  // /** SUBSCRIPTION INTERFACES */

  // pub interface Unsubscribable {
  //   unsubscribe(): void;
  // }

  // pub type TeardownLogic = Unsubscribable | Function | void;

  // pub interface SubscriptionLike extends Unsubscribable {
  //   unsubscribe(): void;
  //   readonly closed: boolean;
  // }

  // pub type SubscribableOrPromise<T> = Subscribable<T> | Subscribable<never> | PromiseLike<T> | InteropObservable<T>;

  // /** OBSERVABLE INTERFACES */

  // pub interface Subscribable<T> {
  //   subscribe(observer?: PartialObserver<T>): Unsubscribable;
  //   /** @deprecated Use an observer instead of a complete callback */
  //   subscribe(next: null | undefined, error: null | undefined, complete: () => void): Unsubscribable;
  //   /** @deprecated Use an observer instead of an error callback */
  //   subscribe(next: null | undefined, error: (error: any) => void, complete?: () => void): Unsubscribable;
  //   /** @deprecated Use an observer instead of a complete callback */
  //   subscribe(next: (value: T) => void, error: null | undefined, complete: () => void): Unsubscribable;
  //   subscribe(next?: (value: T) => void, error?: (error: any) => void, complete?: () => void): Unsubscribable;
  // }

  // pub type ObservableInput<T> = SubscribableOrPromise<T> | ArrayLike<T> | Iterable<T>;

  // /** @deprecated use {@link InteropObservable } */
  // pub type ObservableLike<T> = InteropObservable<T>;

  // pub type InteropObservable<T> = { [Symbol.observable]: () => Subscribable<T>; };

  // /** OBSERVER INTERFACES */

  // pub interface NextObserver<T> {
  //   closed?: boolean;
  //   next: (value: T) => void;
  //   error?: (err: any) => void;
  //   complete?: () => void;
  // }

  // pub interface ErrorObserver<T> {
  //   closed?: boolean;
  //   next?: (value: T) => void;
  //   error: (err: any) => void;
  //   complete?: () => void;
  // }

  // pub interface CompletionObserver<T> {
  //   closed?: boolean;
  //   next?: (value: T) => void;
  //   error?: (err: any) => void;
  //   complete: () => void;
  // }

  // pub type PartialObserver<T> = NextObserver<T> | ErrorObserver<T> | CompletionObserver<T>;

  // pub interface Observer<T> {
  //   closed?: boolean;
  //   next: (value: T) => void;
  //   error: (err: any) => void;
  //   complete: () => void;
  // }

  // /** SCHEDULER INTERFACES */

  // pub interface SchedulerLike {
  //   now(): number;
  //   schedule<T>(work: (this: SchedulerAction<T>, state?: T) => void, delay?: number, state?: T): Subscription;
  // }
  // pub interface SchedulerAction<T> extends Subscription {
  //   schedule(state?: T, delay?: number): Subscription;
  // }

  // pub type ObservedValueOf<O> = O extends ObservableInput<infer T> ? T : never;
// } 

