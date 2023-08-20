// place files you want to import through the `$lib` alias in this folder.

type canteen = {
    id: Number,
    name: String
}

type food = {
    id: number,
    avg: number,
    name: string,
    name_en: string,
    price_student: number
    count_of_rev: number,
    food_id: number,
}

type foodAd = {
    category: string[],
    food_type: number,
    id: number,
    name: string,
    name_en: string,
    price_employee: number,
    price_extern: number,
    price_student: number,
    weight: string
}
