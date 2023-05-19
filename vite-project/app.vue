<template>
<div>
    <span>WELCOME</span>
    <br>
    <div class="suspend_area">
        <Suspense>
            <template #default>
                <MyName></MyName>
            </template>
            <template #fallback>
                loading...
            </template>
        </Suspense>
    </div>
</div>
</template>

<script setup lang="ts">

</script>

<style scoped>
.suspend_area {
    padding: 10px;
    border: 1px solid black;
}

</style>