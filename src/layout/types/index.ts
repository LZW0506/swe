import type {VNode} from "vue";


export interface menuItemType {
    key: string | number,
    icon: VNode | string,
    label: string
}

export interface editMenuItemType extends menuItemType {
    uuid: string
}