// place files you want to import through the `$lib` alias in this folder.

type canteen = {
    id: Number,
    name: String
}

type food = {
    available: boolean,
    avg: number,
    category: string[],
    food_id: number,    
    food_type: number,
    name: string,
    price_student: number
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