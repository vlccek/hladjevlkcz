<script lang="ts">
    export let data;

    let fooddata: foodAd = data.meal;

    console.log(fooddata);
</script>

<div class="container">
{fooddata.name}
</div>
