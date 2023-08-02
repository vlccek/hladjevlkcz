// @ts-ignore
export async function load({ fetch, params }) {

    const fetchData = async (id: number) => {
        const res = await fetch(`http://localhost:5000/api/food_detail/${id}`);

        const f: foodAd[] = await res.json();

        return f;
    }

    return { meal: fetchData(Number(params.mealId)) };
}