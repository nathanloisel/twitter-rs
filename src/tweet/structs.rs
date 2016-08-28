use rustc_serialize::json;
use user;
use error;
use error::Error::InvalidResponse;
use entities;
use common::*;

///Represents a single status update.
#[derive(Debug)]
pub struct Tweet {
    //If the user has contributors enabled, this will show which accounts contributed to this
    //tweet.
    //pub contributors: Option<Contributors>,
    //The location point attached to the tweet, if present.
    //pub coordinates: Option<Coordinates>,
    ///UTC timestamp showing when the tweet was posted, formatted like "Wed Aug 27 13:08:45 +0000
    ///2008".
    pub created_at: String,
    ///If the authenticated user has retweeted this tweet, contains the ID of the retweet.
    pub current_user_retweet: Option<i64>,
    ///Link, hashtag, and user mention information extracted from the tweet text.
    pub entities: TweetEntities,
    ///Extended media information attached to the tweet, if media is available.
    ///
    ///If a tweet has a photo, set of photos, gif, or video attached to it, this field will be
    ///present and contain the real media information. The information available in the `media`
    ///field of `entities` will only contain the first photo of a set, or a thumbnail of a gif or
    ///video.
    pub extended_entities: Option<ExtendedTweetEntities>,
    ///"Approximately" how many times this tweet has been liked by users.
    pub favorite_count: i32,
    ///Indicates whether the authenticated user has liked this tweet.
    pub favorited: Option<bool>,
    //Indicates the maximum `FilterLevel` parameter that can be applied to a stream and still show
    //this tweet.
    //pub filter_level: FilterLevel,
    ///Numeric ID for this tweet.
    pub id: i64,
    ///If the tweet is a reply, contains the ID of the user that was replied to.
    pub in_reply_to_user_id: Option<i64>,
    ///If the tweet is a reply, contains the screen name of the user that was replied to.
    pub in_reply_to_screen_name: Option<String>,
    ///If the tweet is a reply, contains the ID of the tweet that was replied to.
    pub in_reply_to_status_id: Option<i64>,
    ///Can contain a language ID indicating the machine-detected language of the text, or "und" if
    ///no language could be detected.
    pub lang: String,
    //TODO: Is this the user-entered location field?
    //When present, the `Place` that this tweet is associated with (but not necessarily where it
    //originated from).
    //pub place: Option<Place>,
    ///If the tweet has a link, indicates whether the link may contain content that could be
    ///identified as sensitive.
    pub possibly_sensitive: Option<bool>,
    ///If this tweet is quoting another by link, contains the ID of the quoted tweet.
    pub quoted_status_id: Option<i64>,
    ///If this tweet is quoting another by link, contains the quoted tweet.
    pub quoted_status: Option<Box<Tweet>>,
    //"A set of key-value pairs indicating the intended contextual delivery of the containing
    //Tweet. Currently used by Twitter’s Promoted Products."
    //pub scopes: Option<Scopes>,
    ///The number of times this tweet has been retweeted (with native retweets).
    pub retweet_count: i32,
    ///Indicates whether the authenticated user has retweeted this tweet.
    pub retweeted: Option<bool>,
    ///If this tweet is a retweet, then this field contains the original status information.
    ///
    ///The separation between retweet and original is so that retweets can be recalled by deleting
    ///the retweet, and so that liking a retweet results in an additional notification to the user
    ///who retweeted the status, as well as the original poster.
    pub retweeted_status: Option<Box<Tweet>>,
    ///The application used to post the tweet, as an HTML anchor tag containing the app's URL and
    ///name.
    pub source: String, //TODO: this is html, i want to parse this eventually
    ///The text of the tweet.
    pub text: String,
    ///The user who posted this tweet.
    pub user: Box<user::TwitterUser>,
    ///If present and `true`, indicates that this tweet has been withheld due to a DMCA complaint.
    pub withheld_copyright: bool,
    ///If present, contains two-letter country codes indicating where this tweet is being withheld.
    ///
    ///The following special codes exist:
    ///
    ///- `XX`: Withheld in all countries
    ///- `XY`: Withheld due to DMCA complaint.
    pub withheld_in_countries: Option<Vec<String>>,
    ///If present, indicates whether the content being withheld is the `status` or the `user`.
    pub withheld_scope: Option<String>,
}

impl FromJson for Tweet {
    fn from_json(input: &json::Json) -> Result<Self, error::Error> {
        if !input.is_object() {
            return Err(InvalidResponse);
        }

        Ok(Tweet {
            //contributors: Option<Contributors>,
            //coordinates: Option<Coordinates>,
            created_at: try!(field(input, "created_at")),
            current_user_retweet: try!(current_user_retweet(input, "current_user_retweet")),
            entities: try!(field(input, "entities")),
            extended_entities: field(input, "extended_entities").ok(),
            favorite_count: field(input, "favorite_count").unwrap_or(0),
            favorited: field(input, "favorited").ok(),
            //filter_level: FilterLevel,
            id: try!(field(input, "id")),
            in_reply_to_user_id: field(input, "in_reply_to_user_id").ok(),
            in_reply_to_screen_name: field(input, "in_reply_to_screen_name").ok(),
            in_reply_to_status_id: field(input, "in_reply_to_status_id").ok(),
            lang: try!(field(input, "lang")),
            //place: Option<Place>,
            possibly_sensitive: field(input, "possibly_sensitive").ok(),
            quoted_status_id: field(input, "quoted_status_id").ok(),
            quoted_status: field(input, "quoted_status").map(Box::new).ok(),
            //scopes: Option<Scopes>,
            retweet_count: try!(field(input, "retweet_count")),
            retweeted: field(input, "retweeted").ok(),
            retweeted_status: field(input, "retweeted_status").map(Box::new).ok(),
            source: try!(field(input, "source")),
            text: try!(field(input, "text")),
            user: try!(field(input, "user").map(Box::new)),
            withheld_copyright: field(input, "withheld_copyright").unwrap_or(false),
            withheld_in_countries: field(input, "withheld_in_countries").ok(),
            withheld_scope: field(input, "withheld_scope").ok(),
        })
    }
}

fn current_user_retweet(input: &json::Json, field: &'static str) -> Result<Option<i64>, error::Error> {
    if let Some(obj) = input.find(field).and_then(|f| f.as_object()) {
        match obj.get("id").and_then(|o| o.as_i64()) {
            Some(id) => Ok(Some(id)),
            None => Err(InvalidResponse),
        }
    }
    else {
        Ok(None)
    }
}

///Container for URL, hashtag, mention, and media information associated with a tweet.
#[derive(Debug)]
pub struct TweetEntities {
    ///Collection of hashtags parsed from the tweet.
    pub hashtags: Vec<entities::HashtagEntity>,
    ///Collection of financial symbols, or "cashtags", parsed from the tweet.
    pub symbols: Vec<entities::HashtagEntity>,
    ///Collection of URLs parsed from the tweet.
    pub urls: Vec<entities::UrlEntity>,
    ///Collection of user mentions parsed from the tweet.
    pub user_mentions: Vec<entities::MentionEntity>,
    ///If the tweet contains any attached media, this contains a collection of media information
    ///from the tweet.
    pub media: Option<Vec<entities::MediaEntity>>,
}

impl FromJson for TweetEntities {
    fn from_json(input: &json::Json) -> Result<Self, error::Error> {
        if !input.is_object() {
            return Err(InvalidResponse);
        }

        Ok(TweetEntities {
            hashtags: try!(field(input, "hashtags")),
            symbols: try!(field(input, "symbols")),
            urls: try!(field(input, "urls")),
            user_mentions: try!(field(input, "user_mentions")),
            media: field(input, "media").ok(),
        })
    }
}

///Container for extended media information for a tweet.
///
///If a tweet has a photo, set of photos, gif, or video attached to it, this field will be present
///and contain the real media information. The information available in the `media` field of
///`entities` will only contain the first photo of a set, or a thumbnail of a gif or video.
#[derive(Debug)]
pub struct ExtendedTweetEntities {
    ///Collection of extended media information attached to the tweet.
    pub media: Vec<entities::MediaEntity>,
}

impl FromJson for ExtendedTweetEntities {
    fn from_json(input: &json::Json) -> Result<Self, error::Error> {
        if !input.is_object() {
            return Err(InvalidResponse);
        }

        Ok(ExtendedTweetEntities {
            media: try!(field(input, "media")),
        })
    }
}
