export type DsmSets = {
    metadata: DsmMetaData,
    entries: {[key: string]: DsmEntry[]},
}

export type DsmMetaData = {
    name: string,
    version: number,
    subset_version: string | null,
    contributor: string,
    date_created: string,
    description: string,
    url: string,
    year: string,
    formatter: string,
    classes: {[key: number]: DsmClasses},
    licenses: {[key: number]: DsmLicense},
}

export type DsmClasses = {
    class: string,
    subclass: string | null,
}

export type DsmLicense = {
    name: string,
    url: string,
}

export type DataForm = {
    name: string,
    url: string,
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
    file_name: string,
    image_relative_path: string,
    width: number,
    height: number,
    license: number | null,
    flickr_url: string | null,
    coco_url: string | null,
    date_captured: number | null,
    annotation: DsmAnnotation[],
}

export type MergedSet = {
    datasets: DsmSets[],
    name: string,
    version: number,
    class_mapper: ClassMapper,
    license_mapper: LicenseMapper,
    date_created: string,
    description: string,
}

export type ClassMapper = {
    classes: Map<string, number>,
    mapping: Map<string, string[]>,
    custom: Map<string, Custom> | null,
}

export type LicenseMapper = {
    current_id: number,
    licences: Map<number, DsmLicense>,
    mapping: Map<string, Map<number, number>>,
}

export type Custom = {
    mapping: Map<string, string>,
}