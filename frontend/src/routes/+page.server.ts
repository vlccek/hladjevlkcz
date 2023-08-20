// @ts-ignore
export async function load({ fetch }) {
    const res = await fetch(`http://localhost:8000/api/canteens`);
    const canteens: canteen[] = await res.json();
    return {canteens};
}