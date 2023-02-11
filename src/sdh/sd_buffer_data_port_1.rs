#[doc = "Register `sd_buffer_data_port_1` reader"]
pub struct R(crate::R<SD_BUFFER_DATA_PORT_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SD_BUFFER_DATA_PORT_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SD_BUFFER_DATA_PORT_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SD_BUFFER_DATA_PORT_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `sd_buffer_data_port_1` writer"]
pub struct W(crate::W<SD_BUFFER_DATA_PORT_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SD_BUFFER_DATA_PORT_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<SD_BUFFER_DATA_PORT_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SD_BUFFER_DATA_PORT_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cpu_data1` reader - "]
pub type CPU_DATA1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `cpu_data1` writer - "]
pub type CPU_DATA1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u16, SD_BUFFER_DATA_PORT_1_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn cpu_data1(&self) -> CPU_DATA1_R {
        CPU_DATA1_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn cpu_data1(&mut self) -> CPU_DATA1_W<0> {
        CPU_DATA1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Buffer Data Port 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sd_buffer_data_port_1](index.html) module"]
pub struct SD_BUFFER_DATA_PORT_1_SPEC;
impl crate::RegisterSpec for SD_BUFFER_DATA_PORT_1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sd_buffer_data_port_1::R](R) reader structure"]
impl crate::Readable for SD_BUFFER_DATA_PORT_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sd_buffer_data_port_1::W](W) writer structure"]
impl crate::Writable for SD_BUFFER_DATA_PORT_1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets sd_buffer_data_port_1 to value 0"]
impl crate::Resettable for SD_BUFFER_DATA_PORT_1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
