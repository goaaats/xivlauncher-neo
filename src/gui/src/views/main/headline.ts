export interface Headline {
    news: HeadlineEntry[]
    topics: HeadlineEntry[]
    pinned: HeadlineEntry[]
    banner: BannerEntry[]
}

export interface BannerEntry {
    lsb_banner: string
    link: string
}

export interface HeadlineEntry {
    date: string,
    title: string,
    url: string,
    id: string,
    tag: string,
}
