#[derive(Debug)]
enum Tier {
    Gold,
    Silver,
    Platinum,
}

#[derive(Debug)]
enum Subscription {
    Free,
    //  (price_per_month, plan_length_months)
    Basic(f64, u32),
    Premium { tier: Tier },
}

impl Subscription {
    fn summarize(&self) {
        match self {
            Self::Free => println!("You have limited access to the site"),
            Self::Basic(price, months) => println!("You have limited access to the site's premium features for {price} for {months} months"),
            Self::Premium { tier } => println!("You have full access to the site's premium features. Your tier is {:?}", tier),
        }
    }
}

fn main() {
    let subscription1: Subscription = Subscription::Free;
    let subscription2: Subscription = Subscription::Basic(12.99, 6);
    let subscription3: Subscription = Subscription::Premium { tier: Tier::Silver };

    subscription1.summarize();
    subscription2.summarize();
    subscription3.summarize();
}
