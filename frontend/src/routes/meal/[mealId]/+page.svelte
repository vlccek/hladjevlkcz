<script lang="ts">
    import {Icon} from "@smui/button";

    export let data;

    let food_data: foodAd = data.meal;
    console.log(food_data)

</script>

<div class="bg-indigo-darken-4 m-b-xs">
    <div class=" container">
        <h2 class="m-t-none p-t-md m-b-none">
            {food_data.name}
        </h2>
        <h3 class="p-t-none m-t-none">
            {food_data.name_en}
        </h3>
        Hodnocení: <Icon class="material-icons">star</Icon>
    </div>
</div>

<div class="container" >

dalši hovno :)</div>