

export interface menuItemType {
    key: string | number,
    icon:  string,
    label: string
}

export interface editMenuItemType extends menuItemType {
    uuid: string
}