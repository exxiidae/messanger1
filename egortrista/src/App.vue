<script setup lang="ts">
//импортим из вуе две 2 two функции
//onMounted = запускает код после отображения всех компонентов
//ref - быстрая переменая)
import {onMounted ,ref, nextTick } from "vue";

import Database from "@tauri-apps/plugin-sql";

//строим структуру одного сообщения
interface Message{
  id:number;
  author:string;
  body:string;
  created_ad:string;
}
const draft = ref("");

const messages = ref<Message[]>([]);

const status = ref("пидарасыыыы");

//подключение к бд пока его нет использвуем нуул
let db:Database | null=null;
//асинхр функции загрузки сообщений из бд
async function loadMessages(){
  if(!db) return;
  messages.value = await db.select<Message[]>(
      "SELECT id ,author,body,created_at FROM messages ORDER BY id ASC"
  );
}
async function sendMessage(){
  const body = draft.value.trim();
  if(!body) return;
  if(!db) return;
  //добавляем новое сообщение в бд
  await db.execute(
      "INSERT INTO messages (author,body) VALUES ($1,$2)",
      ["Вы", body],
  );
  draft.value = "";
  await loadMessages();
  await nextTick();
  scrollToBottom();
}

//функция скролла вниз
function scrollToBottom() {
  const messagesContainer = document.querySelector('.messages-wrapper');
  if (messagesContainer) {
    messagesContainer.scrollTop = messagesContainer.scrollHeight;
  }
}

onMounted(async ()=>{
  try{
    db= await Database.load("sqlite:messenger.db");
    await loadMessages();
    status.value="local message story";
    await nextTick();
    scrollToBottom();
  }catch (error){
    console.error(error);
    status.value="саси пипиндрик"
  }
});
</script>

<template>
  <main class="app">
    <header class="header">
      <div>
        <h1>dlkfgjh</h1>
        <p>{{status}}</p>
      </div>
      <span class="badge">
        local
      </span>
    </header>
    <section class="chat">
      <div class="chat-infg">
        <h2>1st chat</h2>
        <p>zalupa</p>
      </div>
      <div class="messages-wrapper">
        <div class="messages">
          <div
              v-if="messages.length === 0"
              class="empty"
          >
            <strong>
              123123
            </strong>
            <span>dfghfghfghfrgh</span>
          </div>
          <article
              v-for="message in messages"
              :key="message.id"
              class="message"
          >
            <p>{{message.body}}</p>
            <footer>
              <span>{{message.author}}</span>
              <span>|</span>
              <span>{{message.created_ad}}</span>
            </footer>
          </article>
        </div>
      </div>
      <form
          class="composer"
          @submit.prevent="sendMessage()"
      >
        <input
            v-model="draft"
            type="text"
            placeholder="send message"
            autocomplete="off"
        />
        <button type="submit">сиськи</button>
      </form>
    </section>
  </main>
</template>

<style scoped>
:global(*){
  box-sizing:border-box;
}

:global(html){
  background: #111318;
  color-scheme:dark;
}

:global(body){
  margin: 0;
  font-family:
      Inter,
      system-ui,
      -apple-system,
      BlickMacSystemFont,
      "Segoe UI",
      sans-serif;

  color:#f2f3f5;
  background: #111318;
  height: 100vh;
  overflow: hidden;
}

.app{
  height: 100vh;
  display: flex;
  flex-direction: column;
}

.header{
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 18px 24px;
  border-bottom: 1px solid #292c34;
  background: #17191f;
  flex-shrink: 0;

}
.header h1{
  margin: 0;
  font-size: 18px;
}
.header p{
  margin: 4px 0 0;
  font-size: 12px;
  color: aqua;
  background: aquamarine;

}
.badge{
  padding: 6px 12px;
  border:1px solid #111318;
  border-radius: 6px;
  color:#111318;
  background: blue;
  font-size: 12px;
}
.chat{
  flex:1;
  min-height: 0;
  display: flex;
  flex-direction: column;

}
.chat-infg{
  flex-shrink: 0;
  padding: 20px 25px;
  border-bottom: 1px solid #123543;

}
.chat-infg h2{
  margin:0;
  font-size: 16px;
}
.chat-infg p{
  margin: 5px 0 0;
  color: #3546;

}
.messages-wrapper {
  flex: 1;
  overflow-y: auto;
  min-height: 0;
}

.messages{
  display: flex;
  flex-direction: column;
  gap: 10px;
  padding: 24px;
  padding-bottom: 20px;
}
.empty{
  margin: auto;
  display: flex;
  flex-direction: column;
  gap: 6px;
  text-align: center;
  color:#346346;

}
.message{
  align-self: flex-end;
  max-width: 70%;
  margin: 0;
  padding: 10px 12px;
  border-radius: 10px;
  background: #026;
}
.message p{
  margin: 0;
  line-height: 1.45;
  overflow-wrap:anywhere;

}
.message footer{
  display: flex;
  justify-content: flex-end;
  gap: 5px;
  margin-top:6px ;
  color: #001;
  font-size: 10px;

}
.composer{
  flex-shrink: 0;
  display: flex;
  gap: 10px;
  padding: 16px 20px;
  border-top: 1px solid #221155;
  background: #17191f;
}
.composer input{
  flex: 1;
  min-width: 0;
  padding: 11px 12px;
  border: 1px solid #123543;
  border-radius: 6px;
  outline: none;
  color: #386be0;
  background: #6345;
  font: inherit;
}
.composer input:focus{
  border-color: #4f7fea;

}
.composer button{
  padding: 0 18px;
  border: none;
  border-radius: 8px;
  cursor: pointer;
  color: wheat;
  background: #386be0;
  font:inherit;
  font-weight: 600;
}
.composer button:hover{
  background: #4779e8;
}
</style>