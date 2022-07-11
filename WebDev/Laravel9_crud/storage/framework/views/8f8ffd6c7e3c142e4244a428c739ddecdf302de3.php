<?php $__env->startSection('content'); ?>
	<style>
.container {
	max-width: 450px;
}
.push-top {
	margin-top: 50px;
}
	</style>
	<div class="card push-top">
		<div class="card-header">
			Add user
		</div>
		<div class="card-body">
			<?php if($errors->any()): ?>
				<div class="alert alert-danger">
					<ul>
						<?php $__currentLoopData = $errors->all(); $__env->addLoop($__currentLoopData); foreach($__currentLoopData as $error): $__env->incrementLoopIndices(); $loop = $__env->getLastLoop(); ?>
							<li><?php echo e($error); ?></li>
						<?php endforeach; $__env->popLoop(); $loop = $__env->getLastLoop(); ?>
					</ul>
				</div>
				<br/>
			<?php endif; ?>
			<form method="post" action="<?php echo e(route('students.store')); ?>">
				<div class="form-group">
					<?php echo csrf_field(); ?>
					<label for="name">Name</label>
					<input type="text" class="form-control" name="name"/>
				</div>
				<div class="form-group">
					<label for="email">Email</label>
					<input type="email" class="form-control" name="email"/>
				</div>
				<div class="form-group">
					<label for="phone">Phone</label>
					<input type="tel" class="form-control" name="phone"/>
				</div>
				<div class="form-group">
					<label for="password">Password</label>
					<input type="text" class="form-control" name="password"/>
				</div>
				<button type="submit" class="btn btn-block btn-danger">
					Create User
				</button>
			</form>
		</div>
	</div>
<?php $__env->stopSection(); ?>

<?php echo $__env->make('layout', \Illuminate\Support\Arr::except(get_defined_vars(), ['__data', '__path']))->render(); ?><?php /**PATH /run/media/dany98/Shared/Working/Tutorials/WebDev/VUE_laravel9crud/resources/views/create.blade.php ENDPATH**/ ?>