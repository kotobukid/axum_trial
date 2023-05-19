<script setup lang="ts">
import axios, {type AxiosResponse} from "axios";

const your_name = ref("あああ");

const fetch_name = () => {
    axios.get('/api/name.json').then((res: AxiosResponse<{ name: string }>) => {
        your_name.value = res.data.name
    });
}

const async_dummy_ajax = async () => {
    return new Promise<string>( (resolve) => {
        console.log('A');
        if (true) {
            console.log('B');
            // fetch('/api/name.json').then(async (res) => {
            fetch('http://localhost:3000/api/name').then(async (res) => {
                const r = await res.json();
                console.log(r);
                setTimeout(() => {
                    resolve(r.name);
                }, 1500);
            }).catch((e) => {
                console.log(e)
                resolve('');
            })
        } else {
            console.log('E');
            setTimeout(() => {
                console.log('F');
                resolve('name from dummy');
            }, 3000);
        }
    });
}

your_name.value = await async_dummy_ajax();
</script>

<template lang="pug">
button(@click.prevent="fetch_name") fetch name
span {{ your_name }}
</template>