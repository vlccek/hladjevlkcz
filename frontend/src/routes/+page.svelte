<script lang="ts">
    export let data;
    import { onMount } from "svelte";
    import Select, { Option } from "@smui/select";
    import FoodCard from "$lib/foodcard.svelte";

    let value: number = 1;

    let canteens: canteen[] = data.canteens;

    let foods: food[] = [];

    onMount(async () => await loadFood(value));

    $: value, loadFood(value);

    async function loadFood(id: number) {
        const res = await fetch(
            `http://localhost:5000/api/v2//menues/${id}/today`
        );
        const f = await res.json();
        foods = f;
    }
</script>

<div class="bg-indigo-darken-4 m-b-xs">
    <div class=" container center">
        <Select
            style="width: 300px;"
            key={(canteens) => `${canteens ? canteens.name : ""}`}
            bind:value
            label="Vyberte menzu"
        >
            {#each canteens as canteen (canteen.name)}
                <Option value={canteen.id}>{canteen.name}</Option>
            {/each}
        </Select>
    </div>
</div>
<div class="container">
    {#each foods as food (food.food_id)}
        <FoodCard
            id={food.food_id}
            name={food.name}
            price={food.price_student}
            rating={food.avg}
        />
    {/each}
</div>

<style>
</style>
