<script lang="ts">
    import TopAppBar, { Row, Section, Title } from "@smui/top-app-bar";
    import IconButton from "@smui/icon-button";
    import Button, { Label, Icon } from "@smui/button";
    import LayoutGrid, { Cell } from "@smui/layout-grid";
    import Select, { Option } from "@smui/select";
</script>

<div class="flexy p-none m-none">
    <div class="top-app-bar-container flexor">
        <TopAppBar variant="static">
            <LayoutGrid fixedColumnWidth class="p-none">
                <Section>
                    <Cell>
                        <Button
                            variant="unelevated"
                            class="button-shaped-"
                            href="/"
                        >
                            <Icon class="material-icons">home</Icon>
                        </Button>
                    </Cell>

                    <Cell>
                        <Button
                            variant="unelevated"
                            class="button-shaped-"
                            href="/"
                        >
                            <Label>Hodnocení</Label>
                        </Button>
                    </Cell>

                    <Cell>
                        <Button
                            variant="unelevated"
                            class="button-shaped-"
                            href="/"
                        >
                            <Label>žebříčky</Label>
                        </Button>
                    </Cell>
                </Section>
            </LayoutGrid>
        </TopAppBar>
        <div class="flexor-content">
            <slot />
        </div>
    </div>
</div>
