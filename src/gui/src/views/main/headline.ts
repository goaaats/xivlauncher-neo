export interface Headline {
    news: HeadlineEntry[]
    topics: HeadlineEntry[]
    pinned: HeadlineEntry[]
    banner: BannerEntry[]
}

export interface BannerEntry {
    lsb_banner: String
    link: String
}

export interface HeadlineEntry {
    date: string,
    title: string,
    url: string,
    id: string,
    tag: string,
}
