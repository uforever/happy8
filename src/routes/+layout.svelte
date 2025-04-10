<script>
  import "../app.css";
  import {
    Navbar,
    NavBrand,
    NavLi,
    NavUl,
    NavHamburger,
  } from "flowbite-svelte";

  let { children } = $props();
  import { page } from "$app/state";
</script>

{#if page.url.pathname.startsWith("/chart")}
<!-- <div class="h-screen flex flex-col"> -->
  {@render children()}
<!-- </div> -->
{:else}

<div class="h-screen flex flex-col">
  <Navbar>
    <NavBrand href="/">
      <img src="tauri.svg" class="me-3 h-6 sm:h-9" alt="Flowbite Logo" />
      <span class="self-center whitespace-nowrap text-xl font-semibold dark:text-white">快乐8</span>
    </NavBrand>
    <NavHamburger />
    <NavUl activeUrl={page.url.pathname}>
      <NavLi href="/">查询</NavLi>
      <NavLi href="/add">录入</NavLi>
      <NavLi href="/trend">趋势</NavLi>
    </NavUl>
  </Navbar>

  <div class="bg-lightgray flex-1 p-4">
    {@render children()}
  </div>
</div>

{/if}