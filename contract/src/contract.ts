// Find all our documentation at https://docs.near.org
import { NearBindgen, near, call, view, UnorderedSet, UnorderedMap, LookupMap, assert, NearPromise } from "near-sdk-js";
import { IOrder, IProduct, IProductPreview, IShop, IUser } from "./models";
import { signerAccountId, attachedDeposit, blockTimestamp } from "near-sdk-js/lib/api";
@NearBindgen({})
class EcommerceContract {
    totalProducts: number;
    totalShops: number;
    products: UnorderedMap<IProduct>;
    shopsPerUser: LookupMap<string>;
    shops: UnorderedMap<IShop>;

    productsPerUser: LookupMap<UnorderedSet<IOrder>>;
    productsPerShop: LookupMap<UnorderedSet<string>>;
    constructor() {
        this.totalProducts = 0;
        this.totalShops = 0;
        this.products = new UnorderedMap<IProduct>("products");
        this.shopsPerUser = new LookupMap<string>("shopsPerUser");
        this.shops = new UnorderedMap<IShop>("shops");

        this.productsPerUser = new LookupMap<UnorderedSet<IOrder>>("productsPerUser");
        this.productsPerShop = new LookupMap<UnorderedSet<string>>("productsPerShop");
    }
    // tạo cửa hàng mới
    @call({})
    createShop({ description, name }: Pick<IShop, "description" | "name">) {
        const existingShop = this.shopsPerUser.get(signerAccountId());

        assert(existingShop == null, "Bạn đã có một cửa hàng");
        const shop: IShop = {
            description,
            id: this.totalShops.toString(),
            name,
            owner: signerAccountId(),
            totalProducts: 0,
        };
        this.shopsPerUser.set(signerAccountId(), shop.id);
        this.shops.set(shop.id, shop);
        this.totalShops++;
    }
    // tạo sản phẩm mới cho cửa hàng của nguwoif gọi hàm
    @call({})
    createProduct({ attributes, description, image, price, stock, title }: Omit<IProduct, "id">): void {
        const shopId = this.shopsPerUser.get(signerAccountId());
        assert(shopId != null, "Bạn chưa có cửa hàng");
        const product: IProduct = {
            attributes,
            description,
            id: this.totalProducts.toString(),
            image,
            price,
            shopId,
            stock,
            title,
        };
        this.products.set(product.id, product);
        this.totalProducts++;

        let productsPerShop = this.productsPerShop.get(shopId, {
            reconstructor: UnorderedSet.reconstruct,
        });
        if (productsPerShop == null) {
            productsPerShop = new UnorderedSet<string>("productsPerShop/" + shopId);
        }
        productsPerShop.set(product.id);
        this.productsPerShop.set(shopId, productsPerShop);
        const shop = this.shops.get(shopId);
        this.shops.set(shopId, {
            description: shop.description,
            id: shop.id,
            name: shop.name,
            owner: shop.owner,
            totalProducts: shop.totalProducts + 1,
        });
    }
    // lấy tổng số sản phẩm
    @view({})
    getTotalProducts(): number {
        return this.products.length;
    }
    // lấy tổng số cửa hàng
    @view({})
    getShopByOwner({ accountId }): IShop {
        return this.shops.get(this.shopsPerUser.get(accountId));
    }
    // lấy thông tin cửa hàng
    @call({})
    updateShop({ description, name }: Pick<IShop, "description" | "name">): void {
        const shopId = this.shopsPerUser.get(signerAccountId());
        assert(shopId != null, "Bạn chưa có cửa hàng");
        const shop = this.shops.get(shopId);
        this.shops.set(shopId, {
            description,
            id: shop.id,
            name,
            owner: shop.owner,
            totalProducts: shop.totalProducts,
        });
    }
    // lấy thông tin sản phẩm
    @view({})
    getProducts({ page = 1, limit = 24 }): IProductPreview[] {
        const skip = (page - 1) * limit;
        return this.products
            .keys({
                start: skip,
                limit,
            })
            .filter((key) => key != null)
            .map((key) => {
                const product = this.products.get(key);
                return {
                    id: product.id,
                    image: product.image,
                    price: product.price,
                    shopId: product.shopId,
                    stock: product.stock,
                    title: product.title,
                    shop: this.shops.get(product.shopId),
                };
            });
    }
    // lấy thông tin sản phẩm theo id
    @view({})
    getProductDetails(props: { id: string }): IProduct {
        const product = this.products.get(props.id);
        product.shop = this.shops.get(product.shopId);
        assert(product != null, "Sản phẩm không tồn tại");
        return product;
    }
    // lấy thông tin cửa hàng theo id
    @view({})
    getShop(props: { id: string }): IShop {
        const shop = this.shops.get(props.id);
        assert(shop != null, "Cửa hàng không tồn tại");
        return shop;
    }
    // lấy thông tin sản phẩm theo id cửa hàng
    @view({})
    getShopProducts(props: { id: string }): IProductPreview[] {
        const productsPerShop = this.productsPerShop.get(props.id, {
            reconstructor: UnorderedSet.reconstruct,
        });
        assert(productsPerShop != null, "Cửa hàng không tồn tại");
        return productsPerShop.toArray().map((productId) => {
            const product = this.products.get(productId);
            return {
                id: product.id,
                image: product.image,
                price: product.price,
                shopId: product.shopId,
                stock: product.stock,
                title: product.title,
            };
        });
    }
    // mua sản phẩm
    @call({
        payableFunction: true,
    })
    buyProduct({ id, quantity = 1 }: { id: string; quantity: number }): NearPromise {
        const product = this.products.get(id);
        assert(product != null, "Sản phẩm không tồn tại");
        assert(product.stock >= quantity, "Sản phẩm đã hết hàng");
        const shop = this.shops.get(product.shopId);
        const owner = shop.owner;
        assert(owner != signerAccountId(), "Không thể mua sản phẩm của chính mình");
        const transferAmount = product.price * quantity;
        assert(
            attachedDeposit() == BigInt(transferAmount),
            `Không đủ tiền để mua sản phẩm ( cần ${transferAmount}, bạn đã gửi ${attachedDeposit()})`
        );
        const productsPerUser = this.productsPerUser.get(signerAccountId(), {
            reconstructor: UnorderedSet.reconstruct,
            defaultValue: new UnorderedSet<IOrder>("productsPerUser/" + signerAccountId()),
        });
        // them san pham vao danh sach san pham cua user
        productsPerUser.set({
            createdAt: blockTimestamp().toString(),
            productId: product.id,
            quantity,
        });
        this.productsPerUser.set(signerAccountId(), productsPerUser);
        // tru so luong san pham
        this.products.set(product.id, {
            attributes: product.attributes,
            description: product.description,
            id: product.id,
            image: product.image,
            price: product.price,
            shopId: product.shopId,
            stock: product.stock - quantity,
            title: product.title,
        });
        return NearPromise.new(owner).transfer(attachedDeposit());
    }

    @view({})
    getAllShops() {
        return this.shops.toArray().map((shop) => shop[1]);
    }
    @call({})
    clearProducts() {
        assert(signerAccountId() == "laffy.testnet", "Only owner can clear products");
        this.products = new UnorderedMap<IProduct>("products");
        this.totalProducts = 0;
        this.productsPerShop = new LookupMap<UnorderedSet<string>>("productsPerShop");
    }
    @view({})
    getProductsByUser({ accountId }: { accountId: string }) {
        const productsPerUser = this.productsPerUser.get(accountId, {
            reconstructor: UnorderedSet.reconstruct,
        });
        return productsPerUser.toArray().map((product) => {
            return {
                id: product.productId,
                quantity: product.quantity,
                createdAt: product.createdAt.toString(),
                product: this.products.get(product.productId),
            };
        });
    }
}
