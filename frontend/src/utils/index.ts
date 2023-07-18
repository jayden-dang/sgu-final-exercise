export const convertToNear = (amount: number) => {
    let amountString = amount.toString();
    const amountLength = amountString.length;
    amountString = "0".repeat(24 - amountLength) + amountString;
    return "0" + amountString.slice(0, -24) + "." + amountString.slice(-24);
};
