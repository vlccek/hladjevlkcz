// @ts-ignore
export async function load({ fetch }) {
    const res = await fetch(`http://localhost:5000/api/v2/canteens`);
    const canteens: canteen[] = await res.json();
    return {canteens};
}