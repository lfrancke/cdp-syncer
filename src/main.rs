use scraper::{Html, Selector};

fn main() {
    let url = "https://docs.cloudera.com/cdp-private-cloud-base/7.1.7/runtime-release-notes/topics/xml-maven.html";
    let res = reqwest::blocking::get(url).unwrap();
    let html = res.text().unwrap();

    let document = Html::parse_document(&html);

    let row_selector = Selector::parse("#runtime_7170-551_maven_artifacts__table_7170-551_maven_artifacts tr").unwrap();

    let groupid_selector = Selector::parse("td[headers=\"runtime_7170-551_maven_artifacts__table_7170-551_maven_artifacts__entry__2\"]").unwrap();
    let artifactid_selector = Selector::parse("td[headers=\"runtime_7170-551_maven_artifacts__table_7170-551_maven_artifacts__entry__3\"]").unwrap();
    let version_selector = Selector::parse("td[headers=\"runtime_7170-551_maven_artifacts__table_7170-551_maven_artifacts__entry__4\"]").unwrap();

    for row in document.select(&row_selector) {
        if let Some(groupid) = row.select(&groupid_selector).next() {
            let groupid = groupid.inner_html();
            let artifactid = row.select(&artifactid_selector).next().unwrap().inner_html();
            let version = row.select(&version_selector).next().unwrap().inner_html();
            println!("<dependency>\n  <groupId>{}</groupId>\n  <artifactId>{}</artifactId>\n  <version>{}</version>\n</dependency>", groupid, artifactid, version)
        }
    }
}
