<!-- FoodCard.svelte -->
<script lang="ts">
  import { goto } from "$app/navigation";
  import Card, {    PrimaryAction,  } from "@smui/card";
  import Chip, { Set, LeadingIcon, Text } from "@smui/chips";
  export let id: number;
  export let name: string;
  export let price: number;
  export let rating: number;
</script>

<div class="card-display">
  <Card class="m-b-xs">
    <PrimaryAction on:click={() => goto(`/meal/${id}`)}>
      <div class="d-flex">
        <div class="food scroll p-l-xs">{name}</div>
        <div class="price">
          <Set chips={["one"]} let:chip>
            <Chip {chip}>
              <LeadingIcon class="material-icons" style="margin-right: 0;"
                >attach_money</LeadingIcon
              ><Text>{price} Kč</Text>
            </Chip>
          </Set>
        </div>
        <div class="review">
          <Set chips={["one"]} let:chip>
            <div>
              <Chip {chip}>
                <LeadingIcon class="material-icons">
                  <div
                    class={rating < 3
                      ? "c-bad"
                      : rating < 4
                      ? "c-meh"
                      : "c-good"}
                  >
                    reviews
                  </div></LeadingIcon
                >

                <Text
                  class={rating < 3 ? "c-bad" : rating < 4 ? "c-meh" : "c-good"}
                  >{rating.toFixed(1)}</Text
                >
              </Chip>
            </div>
          </Set>
        </div>
      </div>
    </PrimaryAction>
  </Card>
</div>

<style>
  .d-flex {
    display: flex;
    justify-content: center;
    align-items: center;
  }
  .food {
    width: 100%;
  }
  .scroll {
    overflow-x: auto;
    white-space: nowrap;
    scrollbar-width: none; /* Skryje scrollovací pruh pro moderní prohlížeče (Firefox) */
  }
</style>
