<script setup>
import {useRouter} from 'vue-router'

const router = useRouter()
const props = defineProps({
	lessonId: {
		type: String,
		required: true
	}
})

function completeAndContinue () {
	router.push(`/courses/${course.id}/lessons/${props.lessonId}`)
}
</script>

<template>
	<button class="button primary"
		@click="completeAndContinue">
		Complete and continue
	</button>
</template>
