<script>
  import { invoke } from "@tauri-apps/api/core";
  import {
    Input, Button, Tooltip, Hr, Badge,
    Table, TableBody, TableBodyCell, TableBodyRow, TableHead, TableHeadCell,
  } from 'flowbite-svelte';
  import { SearchOutline, ChevronLeftOutline, ChevronRightOutline } from 'flowbite-svelte-icons';

  import { createPagination, melt } from '@melt-ui/svelte'

  let issue = $state("");
  let page = $state(1);
  let size = $state(5);
  let total = $state(0);
  let records = $state([]);

  async function invokeQuery() {
    invoke("query", { issue, page, size }).then((res) => {
      records = res.records;
      total = res.total;
    }).catch((e) => alert(e));
  }

  invokeQuery();
  
  let {
    elements: { root, pageTrigger },
    states: { pages, range }
  } = $derived(createPagination({
    count: total,
    perPage: size,
    defaultPage: page,
    siblingCount: 1,
    onPageChange: ({ next }) => {
      page = next;
      invokeQuery();
      return next; 
    },
  }));

  function handleSubmit(event) {
    event.preventDefault();
    page = 1;
    invokeQuery();
  }

</script>

<form onsubmit={handleSubmit}>
  <Input id="search" placeholder="期号" size="lg" type="text" bind:value={issue}>
  <!-- <Input id="search" placeholder="最近期数" size="lg" type="number" bind:value={size} min=1> -->
    <SearchOutline slot="left" class="w-6 h-6 text-gray-500 dark:text-gray-400" />
    <Button slot="right" size="sm" type="submit">查询</Button>
  </Input>
  <Tooltip>期号</Tooltip>
</form>

<Hr classHr="my-8" />

<Table noborder>
  <TableHead>
    <TableHeadCell class="text-base">期号</TableHeadCell>
    <TableHeadCell class="text-base">中奖号码</TableHeadCell>
  </TableHead>
  <TableBody>
    {#each records as item}
      <TableBodyRow>
        <TableBodyCell>{item.issue}</TableBodyCell>
        <TableBodyCell class="space-x-1">
          {#each item.winning as num}
            <Badge large border rounded color="red" class="w-6 h-6 font-medium">{num}</Badge>
          {/each}
        </TableBodyCell>
      </TableBodyRow>
    {/each}
  </TableBody>
</Table>

<Hr classHr="my-8" />

<nav
  class="flex flex-col items-center gap-4"
  aria-label="pagination"
  use:melt={$root}
>
  <p class="text-center text-magnum-900">
    共 {total} 条数据, 当前第 {$range.start + 1} - {$range.end} 条
  </p>
  <div class="flex items-center gap-2">
    <button
      class="grid h-8 items-center rounded-md bg-white px-3 text-sm text-magnum-900 shadow-sm
      hover:opacity-75 disabled:cursor-not-allowed disabled:opacity-50 data-[selected]:bg-magnum-900
      data-[selected]:text-red-500"
      use:melt={$pageTrigger({"value": page - 1, "type": "page"})}
      disabled={page === 1}><ChevronLeftOutline /></button
    >
    {#each $pages as page (page.key)}
      {#if page.type === 'ellipsis'}
        <span>...</span>
      {:else}
        <button
          class="grid h-8 items-center rounded-md bg-white px-3 text-sm text-magnum-900 shadow-sm
          hover:opacity-75 disabled:cursor-not-allowed disabled:opacity-50 data-[selected]:bg-magnum-900
        data-[selected]:text-red-500"
          use:melt={$pageTrigger(page)}>{page.value}</button
        >
      {/if}
    {/each}
    <button
      class="grid h-8 items-center rounded-md bg-white px-3 text-sm text-magnum-900 shadow-sm
      hover:opacity-75 disabled:cursor-not-allowed disabled:opacity-50 data-[selected]:bg-magnum-900
    data-[selected]:text-red-500"
      use:melt={$pageTrigger({"value": page + 1, "type": "page"})}
      disabled={page === Math.ceil(total / size)}><ChevronRightOutline /></button
    >
  </div>
</nav>