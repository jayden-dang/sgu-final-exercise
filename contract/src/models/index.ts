import { AccountId } from "near-sdk-js";

export interface IProduct {
    title: string;
    description: string;
    price: number;
    image: string;
    id: string;
    shopId: string;
    stock: number;
    attributes: IProductAttribute;
    shop?: IShop;
}
export type IProductPreview = Omit<IProduct, "description" | "attributes">;

export interface IProductAttribute {
    [key: string]: string;
}

export interface IShop {
    id: string;
    name: string;
    description: string;
    owner: AccountId;
    totalProducts: number;
}

export interface IUser {
    id: AccountId;
    displayName: string;
    ownedProducts: string[];
}

export interface IOrder {
    productId: string;
    quantity: number;
    createdAt: string;
}
