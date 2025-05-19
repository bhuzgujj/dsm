export type DsmSets = {
    metadata: DsmMetaData,
    entries: {[key: string]: DsmEntry[]},
}

export type DsmMetaData = {
    name: String,
    version: number,
    subset_version: String | null,
    contributor: String,
    date_created: String,
    description: String,
    url: String,
    year: String,
    formatter: string,
    classes: {[key: number]: DsmClasses},
    licenses: {[key: number]: DsmLicense},
}

export type DsmClasses = {
    class: String,
    subclass: String | null,
}

export type DsmLicense = {
    name: String,
    url: String,
}

export type DataForm = {
    name: String,
    url: String,
}

export type DsmAnnotation = {
    class: number,
    x: number,
    y: number,
    width: number,
    height: number,
    segmentation: number[],
    iscrowd: number,
    occluded: boolean,
    rotation: number
}

export type DsmEntry = {
    file_name: String,
    image_relative_path: String,
    width: number,
    height: number,
    license: number | null,
    flickr_url: String | null,
    coco_url: String | null,
    date_captured: number | null,
    annotation: DsmAnnotation[],
}