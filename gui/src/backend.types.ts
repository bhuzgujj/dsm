export type DsmSets = {
    metadata: DsmMetaData,
    entries: { [key: string]: DsmEntry[] },
}

export type DsmMetaData = {
    name: string,
    version: string,
    subset_version: string | null,
    contributor: string,
    date_created: string,
    description: string,
    url: string,
    year: string,
    formatter: string,
    classes: { [key: number]: DsmClasses },
    licenses: { [key: number]: DsmLicense },
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
    datasets: { [key: string]: DsmSets[] },
    name: string,
    version: string,
    class_mapper: ClassMapper,
    license_mapper: LicenseMapper,
    date_created: string,
    description: string,
}

export type ClassMapper = {
    classes: { [key: string]: number },
    mapping: { [key: string]: string[] },
    custom: { [key: string]: Custom } | null,
}

export type LicenseMapper = {
    current_id: number,
    licences: { [key: number]: DsmLicense },
    mapping: { [key: string]: { [key: number]: number } },
}

export type Custom = {
    mapping: { [key: string]: string },
}

export type IncludedSet = {
    names: string,
    version: string,
    group: string
}