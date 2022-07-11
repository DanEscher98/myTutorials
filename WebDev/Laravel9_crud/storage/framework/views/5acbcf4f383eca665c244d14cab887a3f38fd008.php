<?php $__env->startSection('content'); ?>
	<style>
.push-top {
	margin-top: 50px;
}
	</style>
	<div class="push-top">
		<?php if(session()->get('success')): ?>
			<div class="alert alert-success">
				<?php echo e(session()->get('success')); ?>

			</div><br/>
		<?php endif; ?>
		<table class="table">
			<thead>
				<tr class="table-warning">
					<td>ID</td>
					<td>Name</td>
					<td>Email</td>
					<td>Phone</td>
					<td>Password</td>
					<td class="text-center">Action</td>
				</tr>
			</thead>
			<?php $__currentLoopData = $student; $__env->addLoop($__currentLoopData); foreach($__currentLoopData as $students): $__env->incrementLoopIndices(); $loop = $__env->getLastLoop(); ?>
				<tbody>
					<td><?php echo e($students->id); ?></td>
					<td><?php echo e($students->name); ?></td>
					<td><?php echo e($students->email); ?></td>
					<td><?php echo e($students->phone); ?></td>
					<td><?php echo e($students->password); ?></td>
					<td class="text-center">
						<a href="<?php echo e(route('students.edit', $students->id)); ?>" class="btn btn-primary btn-sm">Edit</a>
						<form action="<?php echo e(route('students.destroy', $students->id)); ?>" method="post" style="display: inline-block">
							<?php echo csrf_field(); ?>
							<?php echo method_field('DELETE'); ?>
							<button class="btn btn-danger btn-sm" type="submit">Delete</button>
						</form>
					</td>
				</tbody>
			<?php endforeach; $__env->popLoop(); $loop = $__env->getLastLoop(); ?>
		</table>
	</div>
<?php $__env->stopSection(); ?>

<?php echo $__env->make('layout', \Illuminate\Support\Arr::except(get_defined_vars(), ['__data', '__path']))->render(); ?><?php /**PATH /run/media/dany98/Shared/Working/Tutorials/WebDev/VUE_laravel9crud/resources/views/index.blade.php ENDPATH**/ ?>