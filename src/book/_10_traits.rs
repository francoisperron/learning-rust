#[cfg(test)]
mod tests {

    #[test]
    fn traits_are_like_interfaces() {
        let news = NewsArticle { headline: String::from("Doh"), location: String::from("Springfield"), author: String::from("Homer") };
        assert_eq!(news.summarize(), "Doh, by Homer (Springfield)");

        let tweet = Tweet { username: String::from("mburns"), content: String::from("excellent") };
        assert_eq!(tweet.summarize(), "mburns: excellent");
    }

    trait Summary {
        fn summarize(&self) -> String;
    }

    struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
    }

    impl Summary for NewsArticle {
        fn summarize(&self) -> String {
            format!("{}, by {} ({})", self.headline, self.author, self.location)
        }
    }

    struct Tweet {
        pub username: String,
        pub content: String,
    }

    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }

    #[test]
    fn traits_can_have_default_implementation() {
        let news = NewsArticle { headline: String::from("Doh"), location: String::from("Springfield"), author: String::from("Homer") };
        assert_eq!(news.date(), "Today");

        let tweet = Tweet { username: String::from("mburns"), content: String::from("excellent") };
        assert_eq!(tweet.date(), "A minute ago");
    }

    pub trait Date {
        fn date(&self) -> String {
            String::from("Today")
        }
    }

    impl Date for NewsArticle {}

    impl Date for Tweet {
        fn date(&self) -> String {
            String::from("A minute ago")
        }
    }

    #[test]
    fn traits_can_be_used_as_parameters() {
        let tweet = Tweet { username: String::from("mburns"), content: String::from("excellent") };
        let summary = get_summary(&tweet);
        let summary2 = get_summary_2(&tweet);
        assert_eq!(summary, summary2)
    }

    fn get_summary(summary: &impl Summary) -> String {
        summary.summarize()
    }

    fn get_summary_2<T: Summary>(item: &T) -> String {
        item.summarize()
    }

    #[test]
    fn traits_can_bound_generics_using_where_or_plus() {
        let tweet = Tweet { username: String::from("mburns"), content: String::from("excellent") };
        let wow = add_wow_factor(&tweet);
        assert_eq!(wow, "A MINUTE AGO : MBURNS: EXCELLENT!!!");

        let wow = add_wow_factor_2(&tweet);
        assert_eq!(wow, "A MINUTE AGO : MBURNS: EXCELLENT!!!");
    }

    fn add_wow_factor<T: Summary + Date>(item: &T) -> String {
        format!("{date} : {text}!!!", date = item.date(), text = item.summarize()).to_uppercase()
    }

    fn add_wow_factor_2<T>(item: &T) -> String where T: Summary + Date {
        format!("{date} : {text}!!!", date = item.date(), text = item.summarize()).to_uppercase()
    }

    #[test]
    fn traits_can_be_returned() {
        let summary = get_any_summary();
        assert_eq!(summary.summarize(), "anyone: anything")
    }

    fn get_any_summary() -> impl Summary {
        Tweet { username: String::from("anyone"), content: String::from("anything") }
    }

    #[test]
    fn traits_can_impl_other_traits_aka_blanket_implementations() {
        let news = NewsArticle { headline: String::from("Doh"), location: String::from("Springfield"), author: String::from("Homer") };
        assert_eq!(news.to_emoji("🦊"), "🦊 Doh, by Homer (Springfield) 🦊");

        let tweet = Tweet { username: String::from("mburns"), content: String::from("excellent") };
        assert_eq!(tweet.to_emoji("🪫"), "🪫 mburns: excellent 🪫");
    }

    trait ToEmoji {
        fn to_emoji(&self, emoji: &str) -> String;
    }

    impl<T: Summary> ToEmoji for T {
        fn to_emoji(&self, emoji: &str) -> String {
            format!("{emoji} {summary} {emoji}", summary=self.summarize())
        }
    }
}